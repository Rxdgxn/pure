EXAMPLE_OBJ1={}
EXAMPLE_OBJ2={}
function love.load()
end
function love.update()
end
function love.draw()

love.graphics.setColor(love.math.colorFromBytes(255, 0, 0, nil))
love.graphics.rectangle('fill', 100, 100, 100, 100, nil)
love.graphics.draw(player.img, x, y, 0, 1, 1, 0, 32, nil)

end