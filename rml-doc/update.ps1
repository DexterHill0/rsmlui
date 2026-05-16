$env:RMLUI_INCLUDE_DIR = (Resolve-Path "../rsmlui-sys/RmlUi/Include").Path
doxygen ./Doxyfile

python preprocess.py

cd generated/md
moxygen ../xml --classes --noindex --source-root ../../..rsmlui-sys/RmlUi/Include --templates ../../moxygen-templates

cd ../..

python postprocess.py