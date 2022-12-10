<script>
  // agent to connect to backend canister
  import { exchange_rate } from "../../../declarations/exchange_rate/index.js";

  // loading spinner
  import OverlayLoading from "svelte-overlay-loading";
  import { Stretch } from "svelte-loading-spinners";

  import chartjs from "chart.js/auto";
  import { onMount } from "svelte";

  // date picker
  import Flatpickr from "svelte-flatpickr";
  import "flatpickr/dist/flatpickr.css";

  // icons
  import Fa from "svelte-fa";
  import { faSync } from "@fortawesome/free-solid-svg-icons";
  import {
    faCircleInfo,
    faTriangleExclamation,
  } from "@fortawesome/free-solid-svg-icons";

  let ctx;
  let chart;

  let startTime = 1654034400;
  let endTime = 1654812000;

  let loading = false;
  let startup = true;
  let enableWarning = false;
  let missingData = false;

  let xValues = [];
  let yValues = [];

  async function getExchangeRates(start, end) {
    loading = true;
    missingData = false;

    startup = false;

    xValues = [];
    yValues = [];

    const timerange = {
      start: start,
      end: end,
    };
    const ratesWithInterval = await exchange_rate.get_rates(timerange);

    var interval = Number(ratesWithInterval.interval);
    var rates = ratesWithInterval.rates;

    var sortedRates = rates.sort(function (a, b) {
      return Number(a[0]) - Number(b[0]);
    });
    var next = start;
    sortedRates.forEach(function (item, index) {
      var xValue = Number(item[0]);
      var yValue = item[1];

      // handle missing data
      if (next < xValue) {
        xValues.push(next);
        yValues.push(null);

        next = xValue;
        missingData = true;
      }

      xValues.push(xValue);
      yValues.push(yValue);
      next = next + interval;
    });

    loading = false;

    chart.data.labels = xValues;
    chart.data.datasets[0].data = yValues; // update the data
    chart.update(); // notify chart.js to render the new data
  }

  onMount(async () => {
    chart = new chartjs(ctx, {
      type: "line",
      data: {
        labels: xValues,
        datasets: [
          {
            borderColor: "#d3d3d3",
            data: yValues,
            pointRadius: 0,
          },
        ],
      },
      options: {
        animation: {
          duration: 0,
        },
        plugins: {
          legend: {
            display: false,
          },
          tooltip: {
            enabled: true,
            displayColors: false,
            callbacks: {
              title: function (context) {
                return timestampToString(parseInt(context[0].label));
              },
              label: function (context) {
                return "$" + Number(context.parsed.y).toFixed(2);
              },
            },
          },
        },
        scales: {
          y: {
            ticks: {
              callback: function (value, index, ticks) {
                return "$" + Number(value).toFixed(2);
              },
            },
          },
          x: {
            ticks: {
              callback: function (val, index) {
                return index % 2 === 0
                  ? timestampToString(this.getLabelForValue(val))
                  : "";
              },
            },
          },
        },
      },
    });
  });

  function timestampToString(timestamp) {
    var date = new Date(timestamp * 1000);
    var year = date.getFullYear();
    var month = date.getMonth() + 1;
    var day = date.getDate();
    var hours = date.getHours();
    var minutes = "0" + date.getMinutes();
    var seconds = "0" + date.getSeconds();

    return (
      year +
      "-" +
      month +
      "-" +
      day +
      " " +
      hours +
      ":" +
      minutes.substr(-2) +
      ":" +
      seconds.substr(-2)
    );
  }

  // configuration of date picker (flatpickr)
  const options = {
    enableTime: true,
    time_24hr: true,
    mode: "range",
    minDate: "2021-05-11",
    maxDate: new Date(),
    defaultDate: [new Date().fp_incr(-7), new Date().fp_incr(-1)],
    onChange(selectedDates, dateStr) {
      if (selectedDates.length == 2) {
        startTime = Math.floor(selectedDates[0].getTime() / 1000);
        endTime = Math.floor(selectedDates[1].getTime() / 1000);
      }
    },
  };

  function handleSubmit(event) {
    event.preventDefault();
    getExchangeRates(startTime, endTime);
  }
</script>

<canvas bind:this={ctx} id="myChart" />

{#if enableWarning && missingData}
  <div class="alert warning">
    <Fa icon={faTriangleExclamation} /> Parts of the data are missing and are still
    being fetched. Retry a bit later...
  </div>
{/if}

{#if startup}
  <div class="alert info">
    <Fa icon={faCircleInfo} /> Select a time range and update the chart to get started.
  </div>
{/if}

<div class="date-picker">
  <form on:submit={handleSubmit}>
    <Flatpickr {options} name="date" element="#date-picker">
      <div id="date-picker">
        <input type="text" placeholder="Select time range..." data-input />
      </div>
    </Flatpickr>

    <button type="submit">
      <Fa icon={faSync} /> Update
    </button>
  </form>
</div>

{#if loading}
  <OverlayLoading>
    <div class="center">
      <Stretch color="#d3d3d3" />
    </div>
  </OverlayLoading>
{/if}

<style>
  .center {
    margin-top: 200px;
    align-items: center;
    display: flex;
    justify-content: center;
  }

  .alert {
    margin-top: 30px;
    display: inline-block;
    padding: 16px 16px;
    border-radius: 5px;
  }

  .warning {
    color: #842029;
    border-color: #f5c2c7;
    background-color: #f8d7da;
  }

  .info {
    color: #055160;
    border-color: #cff4fc;
    background-color: #b6effb;
  }

  .date-picker {
    margin-top: 30px;
  }

  .date-picker button {
    display: inline-block;
    border-radius: 5px;
    border-style: solid;
    border-width: 2px;
    border-color: #d3d3d3;
    color: #d3d3d3;
    background-color: #161617;
  }

  .date-picker button:hover {
    color: #d3d3d3;
    background-color: #3a3a3a;
  }

  #date-picker {
    display: inline-block;
  }

  #date-picker input {
    padding: 8px 16px;
    margin-left: 15px;
    width: 260px;
    border-radius: 5px;
    border-style: solid;
    border-width: 2px;
    border-color: #d3d3d3;
    color: #d3d3d3;
    background-color: #161617;
    font-size: 15px;
  }
</style>
