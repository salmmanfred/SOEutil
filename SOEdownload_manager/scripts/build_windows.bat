cd ..
if not exist "bin" mkdir bin
deno compile --allow-all --output ./bin/soe_download_manager.exe mod.ts
cd scripts