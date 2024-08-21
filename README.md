# RustEZ Conversion
Image converter built in Rust. The intended use of this converter is to convert images from one format to another. The main formats that will be supported are JPG/JPEG, PNG, and WebP. I plan to add the ability to convert images from HEIC to one of the [supported formats](#supported-conversions), I don't see a need to convert to HEIC. 

### Table of Conents
- [To Do](#to-do)
- [Supported Conversion](#supported-conversions)

## To Do
### Single Image Conversion
- [x] Build GUI frame and minor components with equi
- [x] Allow user to select image from their file browser
- [x] Show filepath when image is selected
- [x] Show preview of image
- [ ] Allow user to select what file type they want to convert the image to
    - [ ] Detect what type of image the target file is
    - [ ] Handle errors for same type conversion
- [ ] Convert image
    - [ ] Converted images should be saved as a copy to the directory the target image comes from unless specified
    - [ ] Converted images should keep the name of the original file but with the new extension unless specified
- [ ] Allow user to select the directory to save the new image to
- [ ] Allow users to change the name of the converted file before conversion

### Batch Conversion
- [ ] Build UI for batch converstion
- [ ] Show file pathnames of all selected files in a selectable list
    - [ ] Allow users to select and remove images from the list by pressing `backspace`/`delete` or `right click` and choose "Remove"
- [ ] Allow user to select what type of format they want to convert to
    - [ ] Loop through each image and detect the initial format and convert to selected format
- [ ] Create folder, in `downloads` by default titled `[date-time]-converions` 
- [ ] Save converted images to that folder
- [ ] Allow users to select the directory to save the new folder to

#### QOL Imporvements to Consider
- Allow the ability to set the quality of the converted image (dependant on the format)

*...more to come*

### Supported Conversions
- JPG/JPEG to PNG
- JPG/JPEG to WebP
- PNG to JPG/JPEG
- PNG to WebP
- WebP to PNG
- WebP to JPG/JPEG
