
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2516(_: S7, _: S1, _: S5) {}
        
        fn test2516() { foo2516(S6, S3, S7, S1, S6); }
    