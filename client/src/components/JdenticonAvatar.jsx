import { useEffect, useRef, useCallback } from 'react';

// Cache per evitare import multipli
let jdenticonModule = null;

const loadJdenticon = async () => {
  if (!jdenticonModule) {
    try {
      jdenticonModule = await import('jdenticon');
    } catch (error) {
      console.error('Errore caricamento jdenticon:', error);
    }
  }
  return jdenticonModule;
};

function JdenticonAvatar({ 
  value = '', 
  size = 100, 
  circular = false,
  borderColor = '#e5e7eb',
  borderWidth = 2
}) {
  const svgRef = useRef(null);

  const updateIdenticon = useCallback(async () => {
    if (!svgRef.current || !value) return;

    try {
      const jdenticon = await loadJdenticon();
      svgRef.current.innerHTML = '';
      jdenticon.update(svgRef.current, value);
    } catch (error) {
      console.error('Errore generazione identicon:', error);
    }
  }, [value]);

  useEffect(() => {
    updateIdenticon();
  }, [updateIdenticon]);

  return (
    <svg
      ref={svgRef}
      width={size}
      height={size}
      className="w-100 h-100"
      style={{
        borderRadius: circular ? '50%' : '8px',
        objectFit: 'cover',
        border: circular ? `${borderWidth}px solid ${borderColor}` : 'none',
        boxShadow: circular ? '0 2px 4px rgba(0, 0, 0, 0.1)' : 'none'
      }}
      data-jdenticon-value={value}
    />
  );
}

export default JdenticonAvatar;