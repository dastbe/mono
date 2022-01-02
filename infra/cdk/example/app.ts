import { App } from 'aws-cdk-lib'
import { HelloStack } from './stack'

const app = new App()

new HelloStack(app, 'HelloStack', {
  env: {
    account: '270025496640',
    region: 'us-west-2'
  }
})

new HelloStack(app, 'HelloStack2', {
  env: {
    account: '270025496640',
    region: 'us-east-1'
  }
})

new HelloStack(app, 'HelloStack3', {
  env: {
    account: '270025496640',
    region: 'us-east-2'
  }
})

app.synth()
