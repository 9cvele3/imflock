* Collect subdirs on start and offer them as potential labels
* Add option to copy, and not to move
* Once moved, update self.images

* Support some kind of descriptor: Vec<u8>
    * CEDD in rust - separate library
    * CEDD - with C interface
    * Triangular matrix to store CEDD similarities
    * Descriptor goes together with similarity function
    * Import descriptor and similarity function from file
        * Use extensions for different descriptors: .descriptor.cedd, .descriptor.magface
* Support grouping of images and moving to separate folders
* Support annotations
* How does it relate to cvat tool ? 
    * cvat is not oss, and not free
* Display best matches for each image 
* Display match score for selected images 
* Display directory hierarchy on the side
* Implement hierarchical clustering
* tSNE python for citulje



