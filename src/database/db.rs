use postgres::{self, Connection, ConnectError, SslMode};
use openssl;

use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::Mutex;

use database::migration;

pub struct ConnectionPool {
    pool: Vec<Connection>,
    available: Mutex<VecDeque<usize>>,
}

pub struct Transaction<'a> {
    pool: &'a ConnectionPool,
    transaction: postgres::Transaction<'a>,
    n: usize,
}

impl ConnectionPool {
    pub fn new(url: &str) -> Result<ConnectionPool, ConnectError> {
        let mut pool = Vec::new();
        let mut available = VecDeque::new();

        let ssl = openssl::ssl::SslContext::new(openssl::ssl::SslMethod::Sslv23).unwrap();
        let ssl = SslMode::Require(ssl);

        for n in (0 .. 10) {
            pool.push(try!(Connection::connect(url, &ssl)));
            available.push_back(n);
        }

        migration::migrate(&pool[0]);

        Ok(ConnectionPool {
            pool: pool,
            available: Mutex::new(available),
        })
    }

    pub fn transaction(&self) -> Transaction {
        let mut available = self.available.lock().unwrap();

        let n = available.pop_front().unwrap();     // FIXME: don't panic if there are not enough connections

        Transaction {
            pool: self,
            transaction: self.pool[n].transaction().unwrap(),
            n: n,
        }
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        let mut available = self.pool.available.lock().unwrap();
        available.push_back(self.n);
    }
}

impl<'a> Deref for Transaction<'a> {
    type Target = postgres::Transaction<'a>;

    fn deref(&self) -> &postgres::Transaction<'a> {
        &self.transaction
    }
}
