﻿using Core.Lexer.Tokenization.Models;
using Core.Meta.Attributes;
using Core.Meta.Interfaces;

namespace Core.Meta
{
    public readonly struct Field : IField
    {
        public Field(string name,
            in TypeBase type,
            in Span span,
            in BaseAttribute? deprecatedAttribute,
            in int constantValue, string documentation)
        {
            Name = name;
            Type = type;
            Span = span;
            DeprecatedAttribute = deprecatedAttribute;
            ConstantValue = constantValue;
            Documentation = documentation;
        }

        public string Name { get; }
        public TypeBase Type { get; }
        public Span Span { get; }
        public BaseAttribute? DeprecatedAttribute { get; }
        public int ConstantValue { get; }
        public string Documentation { get; }
    }
}
