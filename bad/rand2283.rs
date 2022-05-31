
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2283(_: S5, _: S4, _: S3) {}
        
        fn test2283() { foo2283(S1, S2, S4, S5, S7, S8); }
    