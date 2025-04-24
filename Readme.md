ImFlock
==========

This is a simple tool that enables user to easily group images into ImageFolder structure,
and later use it for training or testing.


## Roadmap

* [Done] Collect subdirs on start and offer them as potential labels
* [Done] Add option to copy, and not to move
* [Done] Once moved, update self.images
* Add counter (how much of the images are labeled, how much are left to be labeled)

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



