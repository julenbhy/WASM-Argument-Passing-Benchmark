data = readtable('result.csv');

% Remove the 'Result' column as it contains non-numeric data
data.Result = [];

functions = unique(data.Function);

% Loop through each function to generate plots
for i = 1:length(functions)
    % Filter by function
    func_data = data(strcmp(data.Function, functions{i}), :);
    
    % Group by 'Embedder' and calculate the mean of the metrics
    metrics = varfun(@mean, func_data, 'InputVariables', ...
        {'Runtime_setup', 'Module_load', 'Instantiation', 'Arg_passing', 'Execution', 'Result_retrieve', 'Total_time'}, ...
        'GroupingVariables', 'Embedder');
    
    % Remove unnecessary columns
    metrics.GroupCount = [];
    
    % Transpose the table to have metrics in rows
    metric_names = metrics.Properties.VariableNames(3:end);
    metric_names = strrep(metric_names, 'mean_', ''); % Remove 'mean_' from labels
    embedder_types = metrics.Embedder;
    metric_values = table2array(metrics(:, 3:end))';
    
    % Create the bar plot
    figure;
    b = bar(metric_values);
    set(gca, 'YScale', 'log');
    xlabel('Metrics');
    ylabel('Log(average time (ns))');
    title(['Average Time by Metric for Function: ', functions{i}]);
    set(gca, 'xticklabel', strrep(metric_names, '_', '\_'), 'XTickLabelRotation', 45); % Escape underscores

    % Create the legend and add a title
    lgd = legend(strrep(embedder_types, '_', '\_'), 'Location', 'best'); % Escape underscores in legend
    title(lgd, 'Embedder type');
    
    % Add annotations with the values
    hold on;
    for j = 1:size(metric_values, 2)
        % Get the X positions of the current group of bars
        x_positions = b(j).XEndPoints;
        for k = 1:size(metric_values, 1)
            % Position the text above each bar
            text(x_positions(k), metric_values(k, j) * 1.05, sprintf('%.0f ns', metric_values(k, j)), ...
                'HorizontalAlignment', 'center', 'VerticalAlignment', 'bottom', 'FontSize', 8);
        end
    end
    hold off;
end
