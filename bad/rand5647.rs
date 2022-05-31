
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5647(_: S2, _: S1) {}
        
        fn test5647() { foo5647(S5, S2, S8, S4, S6); }
    