
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14278(_: S1, _: S3, _: S5, _: S6, _: S8) {}
        
        fn test14278() { foo14278(S3, S4, S1, S7, S6, S4, S4); }
    