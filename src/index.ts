import { Webhook } from 'discord-webhook-node'
import * as dotenv from 'dotenv'
import express from 'express'
import { z } from 'zod'

dotenv.config()

if (!process.env.PORT) throw new Error('PORT is not defined')
if (!process.env.WEBHOOK_URL) throw new Error('WEBHOOK_URL is not defined')
if (!process.env.WEBHOOK_AVATAR_URL) throw new Error('WEBHOOK_AVATAR_URL is not defined')

const hook = new Webhook(process.env.WEBHOOK_URL)
const app = express()
app.use(express.urlencoded({ extended: true }))
app.use(express.json())

const ReportBodySchema = z.object({
  type: z.union([z.literal('manual'), z.literal('automatic')]),
  message: z.string(),
  extVersion: z.string(),
})

const cache: Record<string, boolean> = {}
app.post('/report', (req, res) => {
  try {
    const body = ReportBodySchema.parse(req.body)
    
    if (body.type === 'automatic') {
      const msg = `${body.extVersion} - ${body.message}`
      if (cache[msg]) return res.status(200).send('OK')
      cache[msg] = true
    }

    hook.setUsername('WNPRedux Reporter')
    hook.setAvatar(process.env.WEBHOOK_AVATAR_URL as string)
    hook.send(`**Type:** ${body.type}\n**Message:** ${body.message}\n**Version:** ${body.extVersion}`)
    res.status(200).send('OK')
  } catch (e) {
    console.error(e)
    res.status(400).send('Bad Request')
  }
})

app.listen(process.env.PORT, () => console.log(`WNPRedux-Report-Proxy listening on port ${process.env.PORT}`))