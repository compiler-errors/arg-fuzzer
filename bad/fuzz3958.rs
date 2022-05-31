
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3958(_: S1, _: S2) {}
        
        fn test3958() { foo3958(S3, S1, S4, S3, S1); }
    