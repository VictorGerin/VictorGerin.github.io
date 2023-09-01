rm .\dist
trunk build --release
rm .\docs\*
mv .\dist\* .\docs\
git add -all
git commit -a -m "update"
git push