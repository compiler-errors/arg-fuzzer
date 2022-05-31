
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16872(_: S4, _: S3) {}
        
        fn test16872() { foo16872(S7, S4, S7, S1); }
    