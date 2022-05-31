
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2800(_: S1, _: S2, _: S8) {}
        
        fn test2800() { foo2800(S4, S3, S4, S2, S4); }
    