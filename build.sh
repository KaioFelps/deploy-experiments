# install npm dependencies and setup rust or whatever is needed
npm install

# build apps
cargo build --release
npm run build

# make important directories
mkdir -p package/www

# copy important things into it
cp ./target/release/deploy-experiment ./package/
if [ -d "public" ]; then
    cp -a public package
fi

if [ -d "dist" ]; then
    cp -a dist package
fi

cp package-lock.json ./package/
cp package.json ./package/
cp ./www/root.html ./package/www/
cp ./run.sh ./package/

# create a fake ts file so that squarecloud recognizes nodejs
echo "console.log('run')" > ./package/main.ts