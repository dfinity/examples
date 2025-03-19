import React, { useEffect, useState } from 'react';
import '../index.css';
import { backend } from 'declarations/backend';

const App = () => {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [selectedDate, setSelectedDate] = useState(new Date());
  const [monthData, setMonthData] = useState([]);
  const [dayDetail, setDayDetail] = useState(null);
  const [loading, setLoading] = useState(false);
  const [newNoteContent, setNewNoteContent] = useState('');

  useEffect(() => {
    renderCalendar();
  }, [currentDate]);

  useEffect(() => {
    renderDayDetail();
  }, [selectedDate]);

  const showLoading = () => setLoading(true);
  const hideLoading = () => setLoading(false);

  const renderCalendar = async () => {
    showLoading();
    await fetchMonthData(currentDate.getFullYear(), currentDate.getMonth() + 1);
    hideLoading();
  };

  const fetchMonthData = async (year, month) => {
    try {
      const monthData = await backend.getMonthData(year, month);
      setMonthData(monthData);
    } catch (error) {
      console.error(`Error fetching data for ${year}-${month}:`, error);
    }
  };

  const renderDayDetail = async () => {
    showLoading();
    const dateString = `${selectedDate.getFullYear()}-${selectedDate.getMonth() + 1}-${selectedDate.getDate()}`;
    try {
      const data = await backend.getDayData(dateString);
      setDayDetail(data?.length > 0 ? data[0] : { notes: [], onThisDay: null });
    } catch (error) {
      console.error('Error rendering day detail:', error);
    }
    hideLoading();
  };

  const handleFetchOnThisDay = async () => {
    showLoading();
    try {
      const dateString = `${selectedDate.getFullYear()}-${selectedDate.getMonth() + 1}-${selectedDate.getDate()}`;
      const result = await backend.fetchAndStoreOnThisDay(dateString);
      console.log(result);
      renderDayDetail();
    } catch (error) {
      console.error('Error fetching On This Day data:', error);
    }
    hideLoading();
  };

  const handleDayClick = (date) => {
    setSelectedDate(new Date(date));
  };

  const handleToday = () => {
    setCurrentDate(new Date());
    setSelectedDate(new Date());
  };

  const handleCompleteNote = async (noteId) => {
    showLoading();
    const dateString = `${selectedDate.getFullYear()}-${selectedDate.getMonth() + 1}-${selectedDate.getDate()}`;
    try {
      await backend.completeNote(dateString, noteId);
      await renderDayDetail();
      await renderCalendar();
    } catch (error) {
      console.error('Error completing note:', error);
    }
    hideLoading();
  };

  const handleNewNoteChange = (e) => setNewNoteContent(e.target.value);

  const handleAddNote = async () => {
    const content = newNoteContent.trim();
    if (content && selectedDate >= new Date(new Date().setHours(0, 0, 0, 0))) {
      showLoading();
      const dateString = `${selectedDate.getFullYear()}-${selectedDate.getMonth() + 1}-${selectedDate.getDate()}`;
      try {
        await backend.addNote(dateString, content);
        setNewNoteContent('');
        await renderDayDetail();
        await renderCalendar();
      } catch (error) {
        console.error('Error adding note:', error);
      }
      hideLoading();
    }
  };

  const renderCalendarDays = () => {
    const daysInMonth = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 0).getDate();
    const firstDayOfMonth = new Date(currentDate.getFullYear(), currentDate.getMonth(), 1).getDay();
    const daysArray = Array.from({ length: daysInMonth }, (_, index) => index + 1);

    return (
      <div className="calendar-grid">
        {Array.from({ length: firstDayOfMonth }).map((_, index) => (
          <div key={`empty-${index}`} className="empty-day"></div>
        ))}
        {daysArray.map((day) => {
          const date = new Date(currentDate.getFullYear(), currentDate.getMonth(), day);
          const isToday = date.toDateString() === new Date().toDateString();
          const isPast = date < new Date(new Date().setHours(0, 0, 0, 0));
          const isSelected = date.toDateString() === selectedDate.toDateString();

          const dayData = monthData.find((data) => new Date(data[0]).toDateString() === date.toDateString());
          const incompleteNotesCount = dayData ? dayData[1].notes.filter((note) => !note.isCompleted).length : 0;

          return (
            <div
              key={day}
              className={`day ${isToday ? 'today' : ''} ${isPast ? 'past' : ''} ${isSelected ? 'selected' : ''}`}
              onClick={() => handleDayClick(date)}
            >
              <span>{day}</span>
              {incompleteNotesCount > 0 && <span className="note-count-indicator">{incompleteNotesCount}</span>}
            </div>
          );
        })}
      </div>
    );
  };

  const renderDayDetailContent = () => {
    if (!dayDetail) return null;

    const { notes, onThisDay } = dayDetail;
    const isPastDay = selectedDate < new Date(new Date().setHours(0, 0, 0, 0));

    return (
      <div id="day-detail">
        <h2>{selectedDate.toLocaleDateString()}</h2>
        <div className="on-this-day">
          <h3>On This Day</h3>
          {onThisDay && onThisDay.length > 0 ? (
            <div>
              <p>
                {onThisDay[0].title || 'No title'} ({onThisDay[0].year?.toString() || 'No year'})
              </p>
              <a href={onThisDay[0].wikiLink || '#'} target="_blank" rel="noopener noreferrer">
                Read more
              </a>
            </div>
          ) : (
            <button id="fetch-on-this-day" onClick={handleFetchOnThisDay}>
              Get data from the Internet
            </button>
          )}
        </div>

        <div className="notes">
          <h3>Notes</h3>
          {notes.length > 0 ? (
            <ul>
              {notes.map((note, index) => (
                <li key={index} className={note.isCompleted ? 'completed' : ''}>
                  {note.content}
                  {!note.isCompleted && <button onClick={() => handleCompleteNote(note.id)}>Mark Complete</button>}
                </li>
              ))}
            </ul>
          ) : (
            <p>No notes available for this day.</p>
          )}
        </div>

        {!isPastDay && (
          <div className="add-note">
            <input
              type="text"
              id="new-note"
              placeholder="New note"
              value={newNoteContent}
              onChange={handleNewNoteChange}
            />
            <button id="add-note" onClick={handleAddNote}>
              Add Note
            </button>
          </div>
        )}
      </div>
    );
  };

  return (
    <div id="root">
      <h1>Daily Planner</h1>
      <div id="calendar">
        <h2>
          {currentDate.toLocaleString('en-US', {
            month: 'long',
            year: 'numeric'
          })}
        </h2>
        <button id="today" onClick={handleToday}>
          Today
        </button>
        {renderCalendarDays()}
      </div>
      {renderDayDetailContent()}
      {loading && (
        <div className="loading-overlay">
          <div className="loading-spinner"></div>
        </div>
      )}
    </div>
  );
};

export default App;
