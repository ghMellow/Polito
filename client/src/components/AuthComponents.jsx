import { useActionState } from "react";
import { Form, Button, Alert, Card } from 'react-bootstrap';
import { Link } from 'react-router';

function LoginForm(props) {
    const [state, formAction, isPending] = useActionState(loginFunction, {email: '', password: ''});

    async function loginFunction(prevState, formData) {
        const credentials = {
            email: formData.get('email'),
            password: formData.get('password'),
        };

        try {
            await props.handleLogin(credentials);
            return { success: true };
        } catch (error) {
            return { error: 'Login failed. Check your credentials.' };
        }
    }

    return (
        <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '50vh' }}>
            <div style={{ width: '100%', maxWidth: '35%' }}>
                { isPending && <Alert variant="warning">Please, wait for the server's response...</Alert> }
                <Card>
                    <Card.Body>
                        <Card.Title className="text-center mb-4">Login</Card.Title>
                        <Form action={formAction}>
                            <Form.Group controlId='email' className='mb-3'>
                                <Form.Label>Email</Form.Label>
                                <Form.Control type='email' name='email' required />
                            </Form.Group>

                            <Form.Group controlId='password' className='mb-3'>
                                <Form.Label>Password</Form.Label>
                                <Form.Control type='password' name='password' required minLength={6} />
                            </Form.Group>

                            <div className="d-flex gap-2 justify-content-center">
                                <Button type='submit' disabled={isPending} style={{ minWidth: '100px' }}>Login</Button>
                                <Link className='btn btn-danger' to={'/'} disabled={isPending} style={{ minWidth: '100px' }}>Annulla</Link>
                            </div>
                            
                            {state.error && <p className="text-danger">{state.error}</p>}
                        </Form>
                    </Card.Body>
                </Card>
            </div>
        </div>
    );
}

function LogoutButton(props) {
  return <Button variant='outline-danger' onClick={props.logout}>Logout </Button>;
}

export { LoginForm, LogoutButton };