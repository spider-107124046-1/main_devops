pipeline {
  agent { node { label 'docker' } }

  environment {
    IMAGE_PREFIX = "pseudopanda/login-app"
  }

  options {
    skipStagesAfterUnstable()
    timestamps()
    disableConcurrentBuilds()
  }

  triggers {
    // Trigger build via GitHub Webhook (preferred)
    githubPush()

    // OR uncomment below for polling Git repo every 5 minutes
    /*
    pollSCM('H/5 * * * *')
    */
  }

  stages {
    stage('Lint & Test Frontend') {
      agent {
        docker {
          reuseNode true
          image 'node:18-alpine' 
        }
      }
      steps {
        dir('Frontend') {
          sh 'npm ci'
          // sh 'npm run lint' but lint script isnt defined
          sh 'npx eslint . --ext .js,.ts,.jsx,.tsx'
          // no tests in test script so it will fail
          sh 'npm run test -- --passWithNoTests'
        }
      }
    }

    stage('Lint & Test Backend') {
      agent {
        dockerfile {
          reuseNode true
          filename 'Dockerfile.test'
          dir 'Backend'
          additionalBuildArgs '--build-arg RUST_BACKTRACE=1'
        }
      }
      steps {
        dir('Backend') {
          sh 'cargo update time'
          // sh 'cargo fmt -- --check'
          // Not enforcing with -- -D warnings because the project contains some warnings
          sh 'cargo clippy'
          sh 'cargo test -- --quiet'
        }
      }
    }

    stage('Build & Push Docker Images') {
      when { branch 'main' }
      steps {
        script {
          docker.withRegistry('', 'docker-hub') {
            // Build and push frontend image
            dir('Frontend') {
              def frontendImage = docker.build("${IMAGE_PREFIX}-frontend:${env.BUILD_ID}")
              frontendImage.push()
              // Tag and push latest version
              sh "docker tag ${IMAGE_PREFIX}-frontend:${env.BUILD_ID} ${IMAGE_PREFIX}-frontend:latest"
              sh "docker push ${IMAGE_PREFIX}-frontend:latest"
            }

            // Build and push backend image
            dir('Backend') {
              def backendImage = docker.build("${IMAGE_PREFIX}-backend:${env.BUILD_ID}")
              backendImage.push()
              // Tag and push latest version
              sh "docker tag ${IMAGE_PREFIX}-backend:${env.BUILD_ID} ${IMAGE_PREFIX}-backend:latest"
              sh "docker push ${IMAGE_PREFIX}-backend:latest"
            }
          }
        }
      }
    }

    stage('Deploy') {
      when { branch 'main' }
      agent { label 'ssh-agent' }
      steps {
        sshagent(['tn-svm-login-app-deploy']) {
          sh '''
            ssh -o StrictHostKeyChecking=no login-app-deploy@192.168.1.69 "\
              cd /mnt/more/login-app && \
              docker compose pull && \
              docker compose up -d \
            "
          '''
        }
      }
    }
  }

  post {
    always { cleanWs() }
    // Example mailer for failure notification
    // failure {
    //   mail to: 'ilam@10082006.xyz',
    //        subject: "NOTICE: Pipeline failed! ${env.JOB_NAME} #${env.BUILD_NUMBER}",
    //        body: "${env.JOB_NAME} build #${env.BUILD_NUMBER} (<${env.BUILD_URL}>) failed."
    // }
    success {
      // Send success notification to some webhook or use the above mail example to send mail
      echo "Pipeline successfully completed!"
    }
  }
}
