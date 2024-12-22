cargo build --release

# make important directories
mkdir package
mkdir package/www

# copy important things into it
cp ./target/release/deploy-experiment ./package/
cp -a ./public ./package
cp ./package* ./package/
cp ./www/root.html ./package/www/
cp ./run.sh ./package/
cp ./squarecloud.app ./package/

# create a fake ts file so that squarecloud recognizes nodejs
echo "console.log('run')" > ./package/main.ts

npm i
./target/release/deploy-experiment
