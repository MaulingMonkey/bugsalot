@setlocal && pushd "%~dp0.."

cargo about --all-features --workspace generate about.hbs > about.html

@endlocal && popd
