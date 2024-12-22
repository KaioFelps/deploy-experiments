# Deploy Experiment
This repository contains a dummy application. The goal is to use GitHub actions and workflows to achieve
an continuous-integration deployment with Square Cloud.

## Scripts
- **build.sh**  - contains the basic script to build and pack only necessary files and send them to Square Cloud:
    - the generated executable of the web app;
    - the html template read by the web app;
    - the `package.json` and `package-lock.json` files, to install the dependencies;
    - the `dist/` directory with the built Inertia ssr server;
    - the `public/` directory (recursively);
    - the environment variable is set in the host (in this case, a .env file is placed directly in the host file explorer);
- **run.sh**    - the script **Square Cloud** will run on start:
    - install the node dependencies;
    - sets permissions to execute the app executable;
    - runs the executable.

`build.sh` is actually the procedure to be placed in the github workflow and is responsible for building and bundling the application.
