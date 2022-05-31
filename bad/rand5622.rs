
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5622(_: S1, _: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test5622() { foo5622(S5, S1, S2, S4, S6, S1, S2); }
    