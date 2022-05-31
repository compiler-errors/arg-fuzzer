
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16464(_: S6, _: S8) {}
        
        fn test16464() { foo16464(S1, S2, S3, S4, S6, S7); }
    