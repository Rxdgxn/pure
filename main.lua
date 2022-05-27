EXAMPLE_OBJ1={}
EXAMPLE_OBJ2={}
function love.load()
end
function love.update()
end
function love.draw()

love.graphics.setColor(love.math.colorFromBytes(255, 0, 0, nil))
love.graphics.rectangle('fill', 100, 100, 100, 100, nil)


if true then
love.graphics.print("Test1", 200, 200, nil)
else
love.graphics.print("Test2", 200, 200, nil)
end

end