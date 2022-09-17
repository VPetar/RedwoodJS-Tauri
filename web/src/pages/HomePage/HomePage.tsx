import { useState } from 'react'

import { invoke } from '@tauri-apps/api'

import { MetaTags } from '@redwoodjs/web'

const HomePage = () => {
  const [resp, setResp] = useState<string>('')
  const [passed, setPassed] = useState<string>('')
  const greet = async () => {
    setResp(await invoke('greet', { name: passed }))
  }
  const calculate = async () => {
    setResp(await invoke('calculate', { numbers: passed }))
  }
  return (
    <>
      <MetaTags title="Home" description="Home page" />

      <h1>Enter comma separated numbers</h1>
      <input type="text" onChange={(e) => setPassed(e.target.value)} />
      <button onClick={() => greet()} type="button">
        greet
      </button>
      <button onClick={() => calculate()} type="button">
        calculate
      </button>
      <br />
      {resp}
    </>
  )
}

export default HomePage
