
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2865(_: S1, _: S2, _: S6) {}
        
        fn test2865() { foo2865(S4, S2, S5, S1, S4); }
    