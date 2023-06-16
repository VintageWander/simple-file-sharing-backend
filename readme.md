# Simple file sharing backend
This is the backend for my simple file sharing service that I made for my final project, in order to graduate from my university. <br>

<br>

# Tech Stack
1. Backend code written in [`Rust`](https://www.rust-lang.org/).
2. Connects to [`PostgreSQL`](https://www.postgresql.org/)
3. Modeling using [`Prisma`](https://www.prisma.io/)
4. File stored on [`AWS S3`](https://aws.amazon.com/s3/)
5. AWS emulator [`LocalStack`](https://localstack.cloud/) was used to run S3 locally

<br>

# Features list ( implemented )

## User: 
- [x] See all users
- [x] Create new user
- [x] Update user
- [x] Delete user

## Folders:
- [x] See all public folders
- [x] See all personal folders
- [x] See all "shared to me" folders
- [x] Create new folders
- [x] Update folders
- [x] Delete folders
- [ ] Manage folder's collaborators
- [ ] Download folder
- [ ] Manage folders' tags

## Files:
- [x] See all public files
- [x] See all personal files
- [x] See all "shared to me" files
- [x] Create new files
- [ ] Update files
- [ ] Delete files
- [ ] Manage file versions
- [ ] Manage file's collaborators
- [ ] Download file
- [ ] File extensions unrestricted
- [ ] Manage files' tags

## Tags
- [ ] Create new tags
- [ ] Delete tags
- [ ] Manage tags of files or folders

## Collaboration
- [ ] Add collaborator to files
- [ ] Add collaborator to folders
- [ ] Delete collaborators from files
- [ ] Delete collaborators from folders

<br>

# How to deploy locally
## Option 1: Docker-compose stack
I'm currently finish implementing the main features first, and then I'll write a `docker-compose.yml` file to help users' who wants to deploy this app locally
<br> TODO

## Option 2: Running on host machine
### 1. Running the database
If you have a Postgres database already running in the background then great! <br>
However, if you want to run Postgres in a Docker container like I do, run the command below: <br>
```yaml
docker-compose -f postgres.yml up -d 
```

<br>

### 2. Setup AWS emulator LocalStack (or use the real AWS if you want)
Here's the [__link__](https://docs.localstack.cloud/getting-started/installation/#how-to-install-localstack) to install LocalStack on your machine. 

<br>

TLDR: You can install their [__CLI tool__](https://docs.localstack.cloud/getting-started/installation/#localstack-cli), and use it to run the AWS emulator in a Docker container, or just use their [__Docker image__](https://docs.localstack.cloud/getting-started/installation/#docker) <br> 
<br>

Setup your profile with: <br>
```yaml
aws configure --profile default
```
There you will define your self-made `access-key` and `secret-key`, as well as `region`. <br>
Though since the AWS emulator is offline, you can provide any arbitrary `region` or keys that you like

<br>

After you're done, you can access your local AWS S3 through:

```yaml
aws --endpoint-url=http://localhost:4566
```

<br>

Of course in order to not type everything everytime I use the AWS CLI, I `alias`-ed it in my `.zprofile`: 

<br>

```yaml
alias local-s3="aws --endpoint-url=http://localhost:4566 s3"
```

<br>

Everytime that I want to see my resources, I can just use it like this: <br>
```yaml
local-s3 ls
```

<br>

Create your new bucket like this
```yaml
local-s3 mb s3://<bucket-name>
```

<br>

Fill in the fields in the `.env.sample` file
```yaml
# Database
DATABASE_URL = "postgresql://local:password@localhost:8001/local"

# Backend environment variables

# Port is the backend's running port
PORT = 8000
# Origin is the frontend's address, to use with CORS
ORIGIN = 

# Token secrets
ACCESS_TOKEN_SECRET = 
REFRESH_TOKEN_SECRET = 

# AWS S3 tokens
ACCESS_KEY_ID = 
SECRET_ACCESS_KEY = 
BUCKET_NAME = 
REGION = 

```

Rename the file to `.env` and you're done with setting up the environment

<br>

Final command to run: <br>
```yaml
cargo prisma migrate deploy && cargo run --release
```

<br>

Wait for the project to build and connect to it via `Postman` or `ThunderClient` through `http://localhost:8000`

<br>

The steps are quite long, that's why I'm working on a one-command-autostart Docker Compose stack so you don't have to do this manually. <br>
But congrats if you managed to successfully build it 🥳🎉