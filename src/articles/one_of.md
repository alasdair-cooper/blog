# Using `OneOf` With **Blazor**
> 27th April 2024

Union types form an important part of modern languages like Rust and TypeScript. However, C# still lacks such a feature ([not]() for [want]() of [trying]() though). A critical part of the functional toolkit, unions are great for writing concise and expressive code. 

Over time, a few .NET libraries have popped up to address the lack of unions in C#. Some modern ones [even use source generators](dunet). In this article I am going to talk about a much older library: [OneOf]().

## OneOf

A OneOf union is declared with a number of generic parameters `OneOf<T1, .., TN>`. The `OneOf` type implements an implicit conversion for each of these parameters.

```csharp
record Circle(int Radius);   
record Rectangle(int Width, int Height);      

OneOf<Circle, Rectangle> value = new Circle(2);   
// or   
OneOf<Circle, Rectangle> value = new Rectangle(3, 4);
```

The real utility is using these as return types. Consider the following interface:

```csharp
record Circle(int Radius);   
record Rectangle(int Width, int Height);      

interface IIconService   
{   
    OneOf<Circle, Rectangle> GetShape();
}
```

We do not care what `GetShape` does, only that it can return a `Circle` or `Rectangle`. In this hypothetical scenario, we only need the diameter of the shape, so we use `Match` to map the different types to a single value.

```csharp
... continued      

IIconService iconService = new IconService();      

var diameter = iconService.GetShape().Map(
    circle => circle.Radius,
    rectangle => int.Max(rectangle.Width, rectangle.Height)   
);
```

Note the effect of adding another type of shape to the union returned from `GetShape`: the code breaks - we get a squiggle and compile time error. If this was a regular switch expression matching on the children classes of some base class we would not get a compile time error. Depending on whether the discard pattern returned a value or threw an exception we might not receive an error at all.

## Blazor and OneOf

There are multiple ways to use `OneOf` unions with Blazor. One of the best ways to use them is as a parameter type.

This would be the regular pattern for a component with branching behaviour:

```razor
# Home.razor      

@page "/" 

<Shape Value="new Circle(2)"/>
```

```
# Shape.razor      

_shapeRenderer(Value);      

@code {  
    [Parameter] [EditorRequired] public required object Value { get; set; } 
    
    RenderFragment<object> _shapeRenderer = 
        shape => shape switch
        {
            Circle circle => @<div>circle</div>,
            Rectangle rectangle => @<div>rectangle</div>,
            _ => @<div>none</div>
        };
}   
```

What about if we used OneOf instead?

```
# Home.razor

@page "/"

<Shape Value="new Circle(2)"/>      
```

```
# Shape.razor     

_shapeRenderer(Value);      
 
@code {
    [Parameter] [EditorRequired] public required OneOf<Circle, Rectangle> Value { get; set; }         
    
    RenderFragment<OneOf<Circle, Rectangle>> _shapeRenderer = 
        shape => shape.Match(
            circle => @<div>circle</div>,
            rectangle => @<div>rectangle</div>);
}
```

The new code is slightly less verbose. However, the real benefit is that the new code only allows either a circle or a rectangle to be passed in, any other type will result in a squiggly and compile time error. Adding another generic parameter to the union type in `Value` and `_shapeRenderer` will now force you to add an extra parameter to the Match in `_shapeRenderer` before it will compile.

Writing code like this lets us work with the type system to enforce correct code on our current selves, our future selves and our colleagues!