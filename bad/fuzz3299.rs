
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3299(_: S4, _: S5, _: S3) {}
        
        fn test3299() { foo3299(S1, S5, S4, S8, S2); }
    