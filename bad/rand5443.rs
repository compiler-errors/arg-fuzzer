
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5443(_: S6, _: S7, _: S7) {}
        
        fn test5443() { foo5443(S4, S3, S5, S6, S5, S5); }
    