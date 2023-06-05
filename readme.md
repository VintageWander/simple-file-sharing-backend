# Simple file sharing backend
This is the backend for my simple file sharing service that I made for my final project, in order to graduate from my university. <br>

# Tech Stack
1. Backend code written in [`Rust`](https://www.rust-lang.org/).
2. Connects to [`PostgreSQL`](https://www.postgresql.org/)
3. Modeling using [`Prisma`](https://www.prisma.io/)
4. File stored on [`AWS S3`](https://aws.amazon.com/s3/)
5. AWS emulator [`LocalStack`](https://localstack.cloud/) was used to run S3 locally

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

## How to deploy locally
I'm currently finish implementing the main features first, and then I'll write a `docker-compose.yml` file to help users' who wants to deploy this app locally
<br> TODO