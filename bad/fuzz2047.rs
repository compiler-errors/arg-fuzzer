
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2047(_: S6, _: S1, _: S4) {}
        
        fn test2047() { foo2047(S7, S8, S6, S5, S1, S7, S1, S7); }
    