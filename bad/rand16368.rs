
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16368(_: S1, _: S8) {}
        
        fn test16368() { foo16368(S4, S2, S7, S6, S1, S6); }
    