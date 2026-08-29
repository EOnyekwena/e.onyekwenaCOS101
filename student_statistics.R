# Student Statistics Project
# Student scores 
scores <- c(65, 72, 80, 55, 90, 68, 74, 82, 60, 77)
scores
mean(scores)
median(scores)
max(scores)
min(scores)
sd(scores)
summary(scores)
barplot(scores,
        main = "Student Scores",
        xlab = "Student",
        ylab = "Scores")
hist(scores,
     main = "Distribution of Student Scores",
     xlab= "Score")
scores >=70
sum(scores >=70)
mean(scores >= 70)*100
scatter.smooth(scores >=77,
               main = "Distribution of students scoring greater than at least 77",
               xlab = "students",
               ylab = "scores>=77")
scores60 <- c(
  "At least 60" = sum(scores >= 60),
  "Below 60" = sum(scores < 60)
)
scores60
pie(scores60,
    labels = paste0(
      names(scores60),
      " (",
      round(scores60 / sum(scores60) * 100),
      "%)"
    ),
    main = "Pie Chart Distribution of Students Scoring at least 60")



