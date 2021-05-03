# minecraft_rust

Hello, this is my personal project in opengl, recreating main concepts of minecraft in rust. 
Its hard work. Started on 22-03-2021. Main inspiration was the video how a guy made minecraft 
from strach in c using opengl. Looked hard, so i decided to one up and do it in rust, i honestly 
did not know what i was getting myself into. It took 3 weeks to render a cube.

# 17-04-2021

Made a rendering system that uses chunks. Chunks have blocks in them. Rn its just static, meaning chunks are not unloaded when far away. But that is going to be the next thing I work on.

# 24-04-2021

Made a chunk load and unload based on distance to camera. Was harder than expected, had to reconsider how to gererate chunks. I think this feature is going to be an issue in the future.

# 30-04-2021

Due to the massive performance draw of rendering all of those blocks some cutting down on unneeded polygons was needed. I added some features that check if any blocks are next to each block. If the block says it is at the end of a chunk or an air block is next to it then the face of the block shows othervise if there are normal blocks next to it it doesnt render the face. Feature might break later as im not exactly sure about the generation of chunks yet.
![image](https://user-images.githubusercontent.com/50622827/116905576-758ca200-ac3f-11eb-9155-b19dc4dbdac2.png)

# 01-05-2021

Implemented some terrain generation. Found out that my ways of rendering the blocks are not very efficient... maybe a million calls to DrawArray. So will need to work on that a lot. Maybe move to glium instead of gl-rs aswell.
![image](https://user-images.githubusercontent.com/50622827/116905445-470ec700-ac3f-11eb-84d8-e8f7b3926ab7.png)
