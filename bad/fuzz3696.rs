
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3696(_: S8, _: S1) {}
        
        fn test3696() { foo3696(S3, S5, S2, S8, S4, S6); }
    