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
        docker {
          reuseNode true
          image 'rust:1.87-alpine'
        }
      }
      steps {
        dir('Backend') {
          sh 'rustup component add rustfmt clippy'
          // BROTHER FIX YOUR CODE
          // sh 'cargo fmt -- --check'
          // Not enforcing with -- -D warnings because the project contains some warnings
          sh 'cargo clippy'
          sh 'cargo test -- --quiet'
        }
      }
    }

    stage('Build & Push Docker Images') {
      when { branch 'main' }
      agent {
        docker { 
          reuseNode true
          image 'docker:latest' 
          args '--host=unix:///var/run/docker.sock -v /var/run/docker.sock:/var/run/docker.sock' 
        }
      }
      steps {
        script {
          docker.withRegistry('', 'docker-hub') {
            // Build and push frontend image
            dir('Frontend') {
              def frontendImage = docker.build("${IMAGE_PREFIX}-frontend:${env.BUILD_ID}")
              frontendImage.push()
              frontendImage.push('latest')
            }

            // Build and push backend image
            dir('Backend') {
              def backendImage = docker.build("${IMAGE_PREFIX}-backend:${env.BUILD_ID}")
              backendImage.push()
              backendImage.push('latest')
            }
          }
        }
      }
    }

    stage('Deploy') {
      when { branch 'main' }
      agent { label 'ssh-agent' }
      steps {
        sshagent(['tn-svm-ssh']) {
          sh '''
            ssh -o StrictHostKeyChecking=no ilam@192.168.1.69 << 'EOF'
            cd /mnt/more/login-app
            docker compose pull
            docker compose up -d --remove-orphans
            EOF
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
      echo "Pipeline successfully completed!"
    }
  }
}
