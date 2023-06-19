mkdir -p ~/minio/data

docker run \
   -p 9000:9000 \
   -p 9090:9090 \
   --name minio \
   -v ~/minio/data:/data \
   -e "MINIO_ROOT_USER=local" \
   -e "MINIO_ROOT_PASSWORD=password" \
   -d \
   quay.io/minio/minio server /data --console-address ":9090"