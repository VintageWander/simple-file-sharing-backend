# Simple file sharing backend
This is the backend for my simple file sharing service that I made for my final project, in order to graduate from my university. <br>

### I'm happy to report that all of the main functionalities are completed

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
- [x] Manage folder's collaborators
- [x] Download folder
- [x] Manage folders' tags

## Files:
- [x] See all public files
- [x] See all personal files
- [x] See all "shared to me" files
- [x] Create new files
- [x] Update files
- [x] Delete files
- [x] Manage file versions
- [x] Manage file's collaborators
- [x] Download file
- [x] Access to file temporary
- [x] File extensions unrestricted
- [x] Manage files' tags

## Tags
- [x] Create new tags
- [x] Delete tags
- [x] Manage tags of files or folders

## Collaboration
- [x] Add collaborator to files
- [x] Add collaborator to folders
- [x] Delete collaborators from files
- [x] Delete collaborators from folders

<br>

# How to deploy locally
1. Adjust some variables in the [`docker-compose.prod.yml`](./docker-compose.prod.yml) based on your likings 

2. Wait for the `simple-file-sharing` container to finish building

3. After the stack finished building and running successfully. These are the endpoints for checking the storage and database
- Database: <br>
  Access [`http://localhost:5555`](http://localhost:5555) to see the database rows

- By default, the `docker-compose` stack exposes Minio console to manage the files underneath. <br>
  Access [`http://localhost:9090`](http://localhost:9090) to see the console, login to see the buckets and data