
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2318(_: S5, _: S1, _: S3, _: S4) {}
        
        fn test2318() { foo2318(S5, S7, S1, S4, S4); }
    