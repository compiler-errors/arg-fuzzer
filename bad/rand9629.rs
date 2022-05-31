
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9629(_: S6, _: S4, _: S6, _: S3) {}
        
        fn test9629() { foo9629(S1, S2, S5, S6, S7); }
    