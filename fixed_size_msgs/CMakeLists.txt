cmake_minimum_required(VERSION 3.21)
project(fixed_size_msgs)

find_package(ament_cmake REQUIRED)
find_package(rosidl_default_generators REQUIRED)

set(msg_files
    "msg/Image720p.msg"
    "msg/Image1080p.msg"
    "msg/Image4k.msg"
)

rosidl_generate_interfaces(${PROJECT_NAME}
    ${msg_files}
)

ament_export_dependencies(rosidl_default_runtime)

ament_package()