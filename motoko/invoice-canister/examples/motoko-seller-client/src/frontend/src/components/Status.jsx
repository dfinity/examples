import React from "react";
import styled from "styled-components";

const Div = styled.div`
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  margin-bottom: 1rem;

  figure {
    margin-bottom: 0;
  }
`;

const Status = ({ status }) => {
  const [src, alt] = React.useMemo(
    () => (status ? ["/happy.png", "licensed!"] : ["/sad.png", "not licensed"]),
    [status]
  );
  
  if (status === null) return null;

  return (
    <Div id="status">
      <figure>
        <picture>
          <img src={src} alt={alt} />
        </picture>
      </figure>
      <figcaption>
        {status ? "Congrats, " : ""}You are {alt}
      </figcaption>
    </Div>
  );
};

export default Status;
