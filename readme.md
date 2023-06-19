# Simple file sharing backend
This is the backend for my simple file sharing service that I made for my final project, in order to graduate from my university. <br>

<br>

# Tech Stack
1. Backend code written in [`Rust`](https://www.rust-lang.org/).
2. Connects to [`PostgreSQL`](https://www.postgresql.org/)
3. Modeling using [`Prisma`](https://www.prisma.io/)
4. File stored on [`AWS S3`](https://aws.amazon.com/s3/)
5. Local S3 [`Minio`](https://min.io/)

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
- [x] Update files
- [x] Delete files
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

### 2. Configure AWS profile

Setup your profile with: <br>
```yaml
aws configure
```
There you will define your self-made `access-key` and `secret-key`, as well as `aws_region`. <br>
Though since the AWS emulator is offline, you can provide any arbitrary `aws_region` or keys that you like

<br>

After you're done, you can access your local Minio through:

```yaml
aws --endpoint-url=http://localhost:9000
```

<br>

Of course in order to not type everything everytime I use the AWS CLI, I `alias`ed it in my `.zprofile`: 

<br>

```yaml
alias minio="aws s3 --endpoint-url=http://localhost:9000"
```

<br>

### 3. Setup Minio as Local S3 for testing (or use the real AWS S3 if you want)

Here's the [__link__](https://min.io/docs/minio/container/index.html) to deploy Minio in Docker

Or you can use the `minio.zsh` file provided in the project directory to spin up a Minio instance in Docker 


<br>

```yaml
zsh minio.zsh
```

From there, go to `http://localhost:9090` with and then create your new S3 bucket.

<br>

The default credentials in `minio.zsh` are defined as follows:

| username | password |
| -------- | -------- |
| local    | password |

<br>

Create a new `access key` and `secret key` to your new bucket that matches with your pre-configured access and secret keys, defined in the previous step.

<br>

In order to check if the bucket is created or not, run this command (assuming you've `alias`ed the command like I do in the previous step)
```yaml
minio ls
```

<br>

### 4. Adjusting the environment variables

<br>

Fill in the fields in the `.env.sample` file
```yaml
# Database
DATABASE_URL = "postgresql://local:password@localhost:5432/local"

# Backend environment variables

# Port is the backend's running port
PORT = 8000
# Origin is the frontend's address, to use with CORS
ORIGIN = 

# Token secrets
ACCESS_TOKEN_SECRET = 
REFRESH_TOKEN_SECRET = 

# AWS S3 tokens
AWS_ACCESS_KEY_ID = 
AWS_SECRET_ACCESS_KEY = 
AWS_BUCKET_NAME = 
AWS_REGION = 

# If you set MINIO to true, make sure to also provide ENDPOINT

# Minio enable
MINIO = true
# Minio endpoint
ENDPOINT = http://127.0.0.1:9000

```

Rename the file to `.env` and you're done with setting up the environment

<br>

### 5. Start the application

Final command to run: <br>
```yaml
zsh build.zsh
```

<br>

Wait for the project to build and connect to it via `Postman` or `ThunderClient` through `http://localhost:8000`

<br>

### 6. (Optional) View database through Prisma Studio
`cargo prisma studio` currently does not work, therefore you would have to run <br>
```yaml
npx prisma studio
```

Assuming you've already have NodeJS installed

<br>

### 7. (Optional) View `Minio` file storage
Open this address in your browser to view the file storage
```yaml
http://localhost:9090
```

The steps are quite long, that's why I'm working on a one-command-autostart Docker Compose stack so you don't have to do this manually. <br>
But congrats if you managed to successfully build it 🥳🎉