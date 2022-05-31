
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo524(_: S5, _: S6) {}
        
        fn test524() { foo524(S7, S5, S1); }
    