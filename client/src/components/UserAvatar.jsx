import { useState } from 'react';

function UserAvatar({
  size = 120,
  defaultColor = '#333333',
  borderColor = '#e5e7eb',
  borderWidth = 2
}) {
  const [imageError, setImageError] = useState(false);

  const imagePath = '/vite.svg';

  const containerStyle = {
    width: size,
    height: size,
    borderRadius: '50%',
    border: `${borderWidth}px solid ${borderColor}`,
    boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)',
    overflow: 'hidden',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: defaultColor
  };


  if (imageError) {
    return (
      <div style={containerStyle}>
        <span
          style={{
            color: 'white',
            fontSize: size * 0.4,
            fontWeight: 'bold',
            fontFamily: 'system-ui, sans-serif'
          }}
        >
          ?
        </span>
      </div>
    );
  }

  return (
    <div style={containerStyle}>
      <img
        src={imagePath}
        alt="Avatar utente"
        style={{
          width: '100%',
          height: '100%',
          objectFit: 'cover'
        }}
        onError={() => setImageError(true)}
      />
    </div>
  );
}

export default UserAvatar;