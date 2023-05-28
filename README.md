# sv_tools

IEC 61850 Sample Value Network latency tester.

=== Generate a sv_tools Docker container

To generate a Docker container containing sv_tools enter the following command:

```
 $ docker build . --tag sv_tools
```

Next you can export the generated Docker image:

```
 $ docker image save -o sv_tools.tar sv_tools
```

You can import the exported Docker image inside a VM after copying
sv_subscriber.tar inside the VM.

```
 $ docker image load -i sv_tools.tar
```

To create and run the container:

```
docker run \
  --rm \
  --network host \
  --privileged \
  --device=/dev/ptp0 \
  -v $(pwd):$(pwd):Z \
    sv_tools /usr/bin/sv_tools \
       -i [your interface to listen] \
       -o $(pwd)/latency.txt
```

