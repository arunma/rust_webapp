```
sudo apt install apt-transport-https ca-certificates curl gnupg-agent software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/docker-archive-keyring.gpg
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(. /etc/os-release; echo "$UBUNTU_CODENAME") stable"
sudo apt update
sudo apt -y install docker-ce
sudo usermod -aG docker $USER
newgrp docker
docker --version
docker ps
docker run busybox
docker ps -a

docker run --name cookie-db -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres:12.3-alpine

sudo systemctl status docer
sudo systemctl restart docker

id -nG


```

```
sudo apt update && sudo apt install snapd\nsudo snap install dbeaver-ce
wget -O - https://dbeaver.io/debs/dbeaver.gpg.key | sudo apt-key add -
echo "deb https://dbeaver.io/debs/dbeaver-ce /" | sudo tee /etc/apt/sources.list.d/dbeaver.list
sudo apt-get update && sudo apt-get install dbeaver-ce

sudo apt install postgresql-client
cargo install diesel_cli --no-default-features --features postgres
sudo apt install libpg-dev

```
export DATABASE_URL=postgres://postgres:password@localhost

```

#### Diesel and synchronization

Diesel only supports synchronous I/O. If we make a synchronous call to Diesel, the thread that is running the request handler will be blocked. The thread pool will soon be depleted and the server wonn't be able to serve more requests., To mitigate this problem, we can use the `actix_web::web::block()` function. This function takes a blocking function and executes it on a separate thread pool, which is different from the Actix thread pool that executes request handler. The `web::block()` function returns a future that's resolved when the blocking database call finishes.



#### Difference between `App::app_data()` and `App::data()`

Both are for creating states in Actix application. Actix creates a thread pool and runs one App instance per thread, you need to decide if the state needs to be shared across threads.

If you only want local states, which means each thread gets its own state and the states work independently from each other, you can use `App::data()`.

If you can want global state that is shared across all threads, you need to construct a thread_safe pointer (usually an Arc) and `clone` it to all threads. 


### Diesel fantastic documentation


https://diesel.rs/guides/getting-started