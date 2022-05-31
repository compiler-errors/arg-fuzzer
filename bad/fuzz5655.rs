
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5655(_: S1, _: S5, _: S8) {}
        
        fn test5655() { foo5655(S7, S6, S1, S7); }
    